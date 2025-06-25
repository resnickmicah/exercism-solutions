(ns raindrops)

(defn convert
  "Converts a number to its corresponding string of raindrop sounds."
  [num]
  (let [div3 (zero? (mod num 3)) 
        div5 (zero? (mod num 5)) 
        div7 (zero? (mod num 7))]
    (cond 
      (and div3 div5 div7) "PlingPlangPlong"
      (and div3 div5) "PlingPlang"
      (and div3 div7) "PlingPlong"
      (and div5 div7) "PlangPlong"
      div3 "Pling"
      div5 "Plang"
      div7 "Plong"
      :else (str num))))
