(ns collatz-conjecture)

(defn collatz-recursive
  [num num-steps]
  (if (> num 1)
    (if (zero? (mod num 2))
      (collatz-recursive (/ num 2) (inc num-steps))
      (collatz-recursive (inc (* num 3)) (inc num-steps)))
    num-steps)
)

(defn collatz
  "Returns the number of steps it takes to reach 1 according
  to the rules of the Collatz Conjecture."
  [num]
  (collatz-recursive num 0)
)
