node {
  stage('SCM') {
    checkout scm
  }
  stage('SonarQube Analysis') {
    def scannerHome = tool 'SonarScanner';
    withSonarQubeEnv() {
      sh "${scannerHome}/bin/sonar-scanner"
    }
  }
  stage('Quality Gate') {
    steps {
      timeout(time: 5, unit: 'MINUTES') {
        waitForQualityGate abortPipeline: true
      }
    }
  }
}
