 pipeline {
     agent any
     stages {
         stage('Build') {
             steps {
                 bat '''set CARGO_HOME=C:\\Users\\Administrator\\.cargo
                 rustup update nightly
                 cargo build  --release --package rust_server --bin rust_server'''
             }
         }
         stage('Deploy') {
             steps {
                archiveArtifacts artifacts: 'target/release/*.exe', fingerprint: true
             }
         }
     }
 }