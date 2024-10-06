pipeline {
    agent { label 'jenkins_agent' } 
    environment {
        // Extend the PATH to include Cargo binaries
        PATH = "$HOME/.cargo/bin:$PATH"
    }
    stages {
        stage('SCM') {
            steps {
                checkout scm
            }
        }
        stage('Build') {
            steps {
                sh 'cargo build'
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test --verbose'
            }
        }
        
        stage('Clippy') {
            steps {
                sh 'cargo clippy --message-format=json &> clippy-report.json'
            }
        }

        stage('SonarQube Analysis') {
            steps {
                script {
                    def scannerHome = tool 'SonarScanner'
                    withSonarQubeEnv {
                        sh "${scannerHome}/bin/sonar-scanner"
                    }
                }
            }
        }
        stage('Quality gate') {
            steps {
                waitForQualityGate abortPipeline: true
            }
        }
    }
}

