plugins {
    kotlin("jvm") version "2.1.0"
}

group = "com.sarajuhosova.pbt"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))

    val kotest = "5.9.1"
    testImplementation("io.kotest:kotest-runner-junit5:$kotest")
    testImplementation("io.kotest:kotest-assertions-core:$kotest")
    testImplementation("io.kotest:kotest-property:$kotest")
}

tasks.withType<Test>().configureEach {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(21)
}
