import org.apache.spark.sql.SparkSession
import org.apache.spark.ml.feature.CountVectorizer

object Tf {
    def main(args: Array[String]) {
        val spark = SparkSession.builder.appName("Tf App").getOrCreate()
        val df = spark.createDataFrame(Seq(
            (0, Array("b", "c", "d", "e", "f", "g", "a", "a", "a", "a", "c")),
            (1, Array("b", "c", "d", "e", "f", "a", "a", "a", "b", "a")),
            (2, Array("b", "c", "d", "e")),
            (3, Array("b", "c", "d")),
            (4, Array("b", "c")),
            (5, Array("b"))
        )).toDF("id", "words")

        val cvm = new CountVectorizer()
            .setInputCol("words")
            .setOutputCol("features")
            .setVocabSize(3)
            .setMinDF(3)
            .fit(df)

        cvm.transform(df).show(false)
    }
}
