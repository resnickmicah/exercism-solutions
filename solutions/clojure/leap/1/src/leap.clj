(ns leap)

(defn leap-year?
  "Determine if a year is a leap year"
  [year]
  (let [divisible-400 (zero? (mod year 400))
        divisible-4 (zero? (mod year 4))
        divisible-100 (zero? (mod year 100))]
    (or 
     divisible-400 
     (and 
      divisible-4
      (not divisible-100))))
  )
