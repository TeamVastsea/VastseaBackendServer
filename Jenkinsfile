 pipeline {
     agent any
     stages {
         stage('Build') {
             steps {
                 set CARGO_HOME C:\Users\Administrator\.cargo
                 bat 'rustup update nightly'
                 bat 'cargo build --color=always --release --package rust_server --bin rust_server'
             }
         }
         stage('Deploy') {
             steps {
                archiveArtifacts artifacts: '/target/release/*.exe', fingerprint: true
             }
         }
     }
 }