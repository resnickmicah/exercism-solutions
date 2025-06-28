(ns grains (:require [clojure.math :as math]))

(defn square
  "Returns the number of grains on the n-th chessboard square."
  [n]
  (bigint (.pow 2M (- n 1))))

(defn total
  "Returns the total number of grains on the chessboard."
  []
  (apply + (map square (seq (range 1 65)))))
