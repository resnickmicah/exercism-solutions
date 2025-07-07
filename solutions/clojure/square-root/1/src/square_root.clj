(ns square-root)

;; Using heron's method as described on wikipedia:
;; https://en.wikipedia.org/wiki/Square_root_algorithms#Heron's_method

(defn sqrt-recursive
  [x S]
  (let [xint (int x)
        x-squared (* xint xint)
        ;; recursively: x_{n+1} = 1/2(x_n + S/x_n)
        next-x (/ (+ (/ S x) x) 2)]
    (if (= S x-squared)
      xint
      (sqrt-recursive next-x S))))

(defn square-root
  "Calculates a number's square root"
  [S]
  ;; initial estimate x_0: S / 2
  (let [initial (/ S 2)]
    (if (= S 1)
      1
      (sqrt-recursive initial S))))
