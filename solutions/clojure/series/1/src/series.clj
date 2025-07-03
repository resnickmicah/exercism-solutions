(ns series (:require [clojure.string :as str]))

(def empty-string-exc
    (IllegalArgumentException. "series cannot be empty"))

(def too-long-exc 
    (IllegalArgumentException. "slice length cannot be greater than series length"))

(def too-short-exc
    (IllegalArgumentException. "slice length cannot be zero"))

(def negative-len-exc
    (IllegalArgumentException. "slice length cannot be negative"))

(defn slices
  "Returns all contiguous substrings of length n from the string s."
  [s n]
  (cond
    (= (.length s) 0) (throw empty-string-exc)
    (> n (.length s)) (throw too-long-exc)
    (= n 0) (throw too-short-exc)
    (< n 0) (throw negative-len-exc)
    :else (map str/join (partition n 1 s)))
)
