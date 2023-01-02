use chrono::Local;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::SinglePart;
use lettre::transport::smtp::authentication::Credentials;
use mongodb::Collection;
use rand::distributions::Alphanumeric;
use rand::prelude::Distribution;
use simple_log::{debug, error};
use crate::{CONFIG, MONGODB};
use crate::user::{EmailTokenInfo, UserInfo};

impl EmailTokenInfo {
    async fn generate_token(&mut self) -> String {
        let mut rng = rand::thread_rng();
        let token: String = Alphanumeric.sample_iter(&mut rng).take(16).map(char::from).collect::<String>();
        self.token = token;
        let collection: &Collection<EmailTokenInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("email_token");
        collection.insert_one(self.clone(), None).await.unwrap();
        self.token.clone()
    }

    fn send(&self, subject: String, content: String) {
        let email = Message::builder()
            .from(unsafe { CONFIG["mail"]["username"].as_str().unwrap() }.parse().unwrap())
            .to(self.mail_box.clone().parse().unwrap())
            .subject(subject)
            .singlepart(SinglePart::html(content))
            .unwrap();

        let creds = Credentials::new(unsafe { CONFIG["mail"]["username"].as_str().unwrap().to_string() }, unsafe { CONFIG["mail"]["token"].as_str().unwrap().to_string() });
        let mailer = SmtpTransport::relay(unsafe { CONFIG["mail"]["smtpUrl"].as_str().unwrap() })
            .unwrap()
            .credentials(creds)
            .build();
        match mailer.send(&email) {
            Ok(_) => debug!("Register verification email sent to: {}", &self.mail_box),
            Err(e) => error!("Can not send mail: {}", e),
        }
    }
}

impl UserInfo {
    pub async fn send_verify_email(&self) {
        let mut token = EmailTokenInfo {
            username: self.username.clone(),
            mail_box: self.mail_box.clone(),
            token: "".to_string(),
            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            ip: self.ip.clone().unwrap(),
        };

        let content = format!(r###"<span style='position: absolute; left: 30%; width: 450px; height: 450px; background-color: #b0d5df; align-items: center; border-radius: .5rem;'>
            <h1 style='text-align: center; position: relative; top: 20px;'>瀚海工艺</h1>
              <div style='position: relative; width: 410px; text-align: center;left: 20px; top: 30px;'>欢迎您注册瀚海，现在我们需要验证您的邮箱，请点击下面的按钮或复制链接至浏览器中继续。本邮件由系统自动发送，如需帮助，请联系群内管理，不要回复本邮件。</div>
              <a style='position: relative; left: 180px; top: 50px; color: #fff; background-color: #198754; display: inline-block; font-weight: 400; line-height: 1.5; text-align: center; text-decoration: none;
                vertical-align: middle; cursor: pointer; user-select: none; border: 1px solid transparent;padding: .375rem .75rem; font-size: 1rem; border-radius: .25rem; transition: color .15s ease-in-out,background-color .15s ease-in-out,border-color .15s ease-in-out,box-shadow .15s ease-in-out;'
                href='https://{0}/verify.html?token={1}' role="button">验证邮箱</a>
              <div style='position: relative; width: 410px; text-align: center;left: 20px; top: 70px;'>https://{0}/verify.html?token={1}</div>
              <h3 style='position: relative; width: 410px; text-align: center;left: 20px; top: 90px;'>欢迎加入瀚海!</h3>
        </span>"###, unsafe { CONFIG["baseUrl"].as_str().unwrap() }, token.generate_token().await);
        token.send("激活您的账号".to_string(), content);
    }
}