(ns file-cleaner.core
  (:require [clojure.java.io :as io]
            [clojure.string :as str])
  (:gen-class))

(defn clean-line [line]
  (let [trimmed (str/trim line)]
    (when-not (str/blank? trimmed)
      trimmed)))

(defn numbered-line [line-number line]
  (str line-number ": " line))

(defn make-output-name [input-name]
  (let [dot-pos (.lastIndexOf input-name ".")]
    (if (neg? dot-pos)
      (str input-name "_resulting_output.txt")
      (str (subs input-name 0 dot-pos) "_resulting_output.txt"))))

(defn transform-lines
  ([lines]
   (transform-lines lines clean-line numbered-line))
  ([lines clean-fn format-fn]
   (->> lines
        (keep clean-fn)
        (map-indexed (fn [idx cleaned]
                       (format-fn (inc idx) cleaned))))))

(defn write-lines! [writer lines]
  (doseq [line lines]
    (.write writer (str line "\n"))))

(defn rewrite-file
  ([input-name]
   (rewrite-file input-name clean-line numbered-line))
  ([input-name clean-fn format-fn]
   (let [output-name (make-output-name input-name)]
     (try
       (with-open [reader (io/reader input-name)
                   writer (io/writer output-name)]
         (write-lines! writer
                       (transform-lines (line-seq reader) clean-fn format-fn))
         {:status :ok :output output-name})
       (catch java.io.IOException e
         {:status :error
          :message (.getMessage e)})))))

(defn -main [& _args]
  (println (rewrite-file "input.txt")))