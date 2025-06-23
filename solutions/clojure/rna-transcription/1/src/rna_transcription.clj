(ns rna-transcription (:require [clojure.string :as str]))

(def rna-complement 
  {
   \G \C
   \C \G
   \T \A
   \A \U
  })

(defn to-rna
  "Returns the RNA complement of the given DNA string sequence."
  [dna]
  (str/join (map #(str (rna-complement %)) dna))
  )
