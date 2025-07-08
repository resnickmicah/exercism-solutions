(ns nucleotide-count)

(def valid-nucleotides (apply hash-set "GTCA"))

(defn count-of-nucleotide-in-strand [nucleotide strand]
  (if (contains? valid-nucleotides nucleotide)
    (count (filter #(= nucleotide %) strand))
    (throw (IllegalArgumentException. (str "Invalid nucleotide: " nucleotide)))))

(defn nucleotide-counts [strand]
  (let [strand-nts (apply hash-set strand)
        nt-counts-set (map #(count-of-nucleotide-in-strand % strand) valid-nucleotides)
        nt-counts-hm (zipmap valid-nucleotides nt-counts-set)]
    (if (clojure.set/subset? strand-nts valid-nucleotides)
      nt-counts-hm
      (throw (IllegalArgumentException. (str "Invalid strand: " strand))))))
