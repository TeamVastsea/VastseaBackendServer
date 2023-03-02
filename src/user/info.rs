use jwt_simple::prelude::{Claims, Duration, HS256Key, MACLike};
use base64::{Engine};
use mongodb::bson::doc;
use mongodb::Collection;
use crate::{CONFIG, MONGODB};
use crate::user::{UserInfo, UserMCProfile};
use crate::user::microsoft::{request_access_token};
use crate::user::minecraft::{get_user_profile, login_with_xbox, user_has_game};
use crate::user::xbox::{xbl_authenticate, xsts_authenticate};

impl UserMCProfile {
    pub async fn from_code(code: String) -> Result<UserMCProfile, String> {
        let response = match request_access_token(code).await {
            Ok(a) => a,
            Err(err) => { return Err(err); }
        };
        UserMCProfile::complete_login(response.access_token.unwrap()).await
    }

    pub async fn from_access_token(access_token: String) -> Result<UserMCProfile, String> {
        return match UserMCProfile::complete_login(access_token).await {
            Ok(a) => Ok(a),
            Err(e) => Err(e)
        };
    }

    async fn complete_login(access_token: String) -> Result<UserMCProfile, String> {
        let xbl_response = match xbl_authenticate(access_token, true).await {
            Ok(a) => a,
            Err(e) => {
                return Err(format!("Cannot get xbl ({})", e));
            }
        };
        let xsts_response = match xsts_authenticate(xbl_response).await {
            Ok(a) => a,
            Err(e) => {
                return Err(format!("Cannot get xsts ({})", e));
            }
        };
        let xbox_token = match login_with_xbox(xsts_response.user_hash, xsts_response.token).await {
            Ok(a) => a,
            Err(e) => {
                return Err(format!("Cannot login xbox ({})", e));
            }
        };
        let has_game = match user_has_game(xbox_token.clone()).await {
            Ok(a) => a,
            Err(e) => {
                return Err(format!("Cannot examine whether has game ({})", e));
            }
        };
        if !has_game {
            return Err("User does not own Minecraft".to_string());
        }
        return match get_user_profile(xbox_token.clone()).await {
            Ok(a) => Ok(a),
            Err(e) => Err(e.to_string())
        };
    }

    // pub fn from_user_info(info: UserInfo) -> UserMCProfile {
    //     UserMCProfile {
    //         uuid: info._id,
    //         user_name: info.display_name,
    //     }
    // }
}

impl UserInfo {
    pub async fn from_mc_profile(mc_profile: UserMCProfile) -> Result<UserInfo, String> {
        let collection: &Collection<UserInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
        match collection.find_one(doc! {"_id": &mc_profile.uuid}, None).await.unwrap() {
            Some(a) => {
                if !a.enabled {
                    return Err("User disabled.".to_string());
                }
                Ok(a)
            }
            None => {
                let info = UserInfo {
                    bind_qq: None,
                    _id: mc_profile.uuid,
                    display_name: mc_profile.user_name,
                    enabled: true,
                    group: vec![unsafe { &CONFIG["defaultUserGroup"] }.as_str().unwrap().to_string()],
                };
                match info.register().await {
                    Ok(_) => Ok(info),
                    Err(err) => Err(err),
                }
            }
        }
    }

    pub async fn register(&self) -> Result<(), String> {
        let collection: &Collection<UserInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
        match collection.insert_one(self, None).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn to_token(&self) -> String {
        let key = HS256Key::from_bytes(base64::engine::general_purpose::STANDARD.decode(unsafe { CONFIG["tokenKey"].as_str().unwrap() }).unwrap().as_slice());
        let claim = Claims::with_custom_claims(self.clone(), Duration::from_days(7));
        key.authenticate(claim).unwrap()
    }

    pub async fn from_token(token: String) -> Result<UserInfo, String> {
        let key = HS256Key::from_bytes(base64::engine::general_purpose::STANDARD.decode(unsafe { CONFIG["tokenKey"].as_str().unwrap() }).unwrap().as_slice());
        let claims = key.verify_token::<UserInfo>(&token, None);
        match claims {
            Ok(info) => Ok(info.custom),
            Err(_) => Err("Cannot parse token or token is valid".to_string())
        }
    }
}