(ns squeaky-clean
  (:require [clojure.string :as str]))

(defn capitalize-first
  "Replace only first character in string with capitalized version"
  [s]
  (str/replace-first s #"([^\p{Lu}])" #(str/upper-case (first %1)))
)

(defn clean-kebab
  "Replaces kebab-case with camelCase"
  [s]
  (if (str/includes? s "-")
    (let [[fs & ah] (str/split s, #"-")]
    map
    (str/join (into [fs] (map capitalize-first ah))))
    s
  )
)

(defn clean
  "Replaces spaces with underscores and control characters with 'CTRL'"
  [s]
  (-> s 
    (str/replace #" " "_")
    (str/replace #"[\u0000-\u001F\u007F-\u009F]" "CTRL")
    clean-kebab
    (str/replace #"[^\p{IsAlphabetic}_]" "")
    (str/replace #"[\p{InGreek}&&[^\p{Lu}]]" "")
  )
)
