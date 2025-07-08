(ns sum-of-multiples (:use [clojure.set]))



(defn sum-of-multiples
  "Calculate the sum of multiples"
  [factors limit]
  (let [multiples (for [factor factors] (apply hash-set (range factor limit factor)))
        combined-multiples (apply clojure.set/union multiples)]
    (apply + combined-multiples)))
