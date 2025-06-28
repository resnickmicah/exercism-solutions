(ns word-count (:require [clojure.string :as str]))

(defn count-one-word
  [needle haystack]
  (count (filter #(= needle %) haystack))
)

(defn word-count
  "Counts how many times each word occurs in the given string"
  [s]
  (let [split-words (filter #(not (or (str/blank? %) (= "'" %))) (str/split s #"(^'|'$|\W+'|'\W+|[\W&&[^']])"))
        all-lowercase (map str/lower-case split-words)
        unique-words (apply hash-set all-lowercase)
        word-counts (map #(count-one-word % all-lowercase) unique-words)]
    (zipmap unique-words word-counts)
  )
)
