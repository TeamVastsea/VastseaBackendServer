 pipeline {
     agent any
     stages {
         stage('Build') {
             steps {
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