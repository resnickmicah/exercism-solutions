(ns eliuds-eggs
  (:require [clojure.math :as math]))

(defn log2 
  [n]
  (/ (math/log n) (math/log 2)))

  (defn bit-length
    [n]
    (int (if (= n 0) 
           0
           (inc (math/floor (log2 n))))))

(defn egg-count
  "Returns the number of 1 bits in the binary representation of the given number."
  [number]
  ;; function body
  (count (filter identity (map #(bit-test number %) (range (bit-length number))))))
