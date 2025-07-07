(ns grade-school)

(defn add [school name grade]
  (let [g (get school grade [])]
    (assoc school grade (conj g name))))

(defn grade [school grade]
    (get school grade []))

(defn sorted [school]
  (let [sorted-grades (for [[k v] school] {k (sort v)})]
    (into (sorted-map) sorted-grades)))
