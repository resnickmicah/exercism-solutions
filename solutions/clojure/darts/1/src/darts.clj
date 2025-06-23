(ns darts (:require [clojure.math :as math]))

(defn score
  "Calculates the score of a dart throw"
  [x y]
  ;; function body
  (let [radius (math/hypot x y)]
    (cond 
      (<= radius 1) 10
      (<= radius 5) 5
      (<= radius 10) 1
      :else 0
    )
  )
)
