(ns pangram (:require [clojure.string :as str]))

(def alphabet "abcdefghijklmnopqrstuvwxyz")

(defn pangram?
  "Returns true if the given string is a pangram; otherwise, returns false"
  [s]
  (->> s
    (#(str/split % #"[\W_0-9]+"))
    (map str/lower-case)
    (str/join "")
    set
    (#(= (set alphabet) %))))
