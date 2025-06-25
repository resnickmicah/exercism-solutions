(ns strain)

(defn retain
  "Keeps the items in coll for which (pred item) returns true."
  [pred coll]
  (filter #(pred %) coll)
  )

(defn discard
  "Removes the items in coll for which (pred item) returns true."
  [pred coll]
  (filter #(not (pred %)) coll)
  )
