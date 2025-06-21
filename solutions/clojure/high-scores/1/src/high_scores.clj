(ns high-scores)

(defn scores
  "Returns all scores"
  [scores]
  ;; function body
  scores
  )

(defn latest
  "Returns the latest score"
  [scores]
  ;; function body
  (last scores)
  )

(defn personal-best
  "Returns the top (highest) score"
  [scores]
  (apply max scores)
  )

(defn personal-top-three
  "Returns the top three scores"
  [scores]
  (take 3 (sort > scores))
  )
