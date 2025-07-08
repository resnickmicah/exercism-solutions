(ns isogram (:require [clojure.string :as str]))

(defn isogram?
  "Returns true if the given string is an isogram; otherwise, returns false"
  [s]
  (as-> s $
    (str/lower-case $)
    (str/replace $ #"\W+" "")
    (frequencies $)
    (vals $)
    (every? #(= % 1) $))
)
