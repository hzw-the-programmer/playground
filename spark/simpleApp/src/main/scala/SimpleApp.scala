import org.apache.spark.sql.SparkSession

// sbt package
// spark-submit --class "SimpleApp" target/scala-2.12/simple-project_2.12-1.0.jar

object SimpleApp {
    def main(args: Array[String]) {
        val spark = SparkSession.builder.appName("Simple Application").getOrCreate()
        val log = spark.read.textFile("/tmp/log").cache()
        val numAs = log.filter(line => line.contains("a")).count()
        val numBs = log.filter(line => line.contains("b")).count()
        println(s"Lines with a: $numAs, Lines with b: $numBs")
        spark.stop()
    }
}