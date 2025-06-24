(ns triangle)

(defn is-valid-triangle?
  "Returns true if triangle with sides a, b, and c satisfies the triangle inequality, and all sides are greater than zero"
  [a b c]
  (let [max-side (max a b c)]
    (and 
      (cond 
        (= max-side a) (>= (+ b c) a)
        (= max-side b) (>= (+ a c ) b)
        (= max-side c) (>= (+ a b ) c))
    (every? #(< 0 %) [a b c]))))
    

(defn equilateral?
  "Returns true if the triangle with sides a, b, and c is equilateral; otherwise, returns false"
  [a b c]
  (and 
    (is-valid-triangle? a b c)
    (= a b c)))

(defn isosceles?
  "Returns true if the triangle with sides a, b, and c is isosceles; otherwise, returns false"
  [a b c]
  (and
   (is-valid-triangle? a b c)
   (or (= a b) (= a c) (= b c))))

(defn scalene?
  "Returns true if the triangle with sides a, b, and c is scalene; otherwise, returns false"
  [a b c]
  (and (is-valid-triangle? a b c)
       (not= a b) (not= a c) (not= b c)))
