(ns cars-assemble)

(defn production-rate
  "Returns the assembly line's production rate per hour,
   taking into account its success rate"
  [speed]
  (let [success-factor 
    (cond
      (and (>= speed 0) (<= speed 4)) 1.0
      (and (>= speed 5) (<= speed 8)) 0.9
      (= speed 9) 0.8
      (= speed 10) 0.77
      :else "invalid")
  ] (* speed 221 success-factor))
  )

(defn working-items
  "Calculates how many working cars are produced per minute"
  [speed]
  (int (/ (production-rate speed) 60))
  )
