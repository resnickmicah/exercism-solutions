(ns etl (:require [clojure.string :as str]))

(defn xf-caps
  [[score letters]]
  (let [lowercase-letter (str/split (str/lower-case (str/join letters)) #"")]
    (zipmap lowercase-letter (repeat score))))

(defn transform [source] (into {} (map xf-caps source)))
