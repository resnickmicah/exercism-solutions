(ns hamming)

(defn distance
  "Returns the hamming distance between two DNA strands."
  [strand1 strand2]
  (if (= (count strand1) (count strand2))
    (apply + (map (fn [c1 c2] (if (= c1 c2) 0 1)) strand1 strand2))
    (throw (IllegalArgumentException. "strands must be of equal length"))))
