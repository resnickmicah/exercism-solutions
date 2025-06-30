(ns complex-numbers (:require [clojure.math :as math]))

(defn real [[a b]] ;; <- arglist goes here
  a
)

(defn imaginary [[a b]] ;; <- arglist goes here
  b
)

(defn abs [[a b]] ;; <- arglist goes here
  (math/sqrt (+ (* a a) (* b b)))
)

(defn conjugate [[a b]] ;; <- arglist goes here
  [a (* -1 b)]
)

(defn add [[a b] [c d]] ;; <- arglist goes here
  [(+ a c) (+ b d)]
)

(defn sub [[a b] [c d]] ;; <- arglist goes here
  (add [a b] [(* -1 c) (* -1 d)])
)

(defn mul [[a b] [c d]] ;; <- arglist goes here
  (let [new-real (- (* a c ) (* b d))
        new-imaginary (+ (* b c) (* a d))]
    [new-real new-imaginary])
)

(defn div [[a b] [c d]] ;; <- arglist goes here
  (let [cd-prod-sum (+ (* c c) (* d d))
        new-real (/ (+ (* a c) (* b d)) cd-prod-sum)
        new-imaginary (/ (- (* b c) (* a d)) cd-prod-sum)] 
    [new-real new-imaginary])
)
