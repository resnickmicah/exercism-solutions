(ns space-age)

(def earth-year-seconds 31557600)

(def periods 
  {:mercury 0.2408467
 :venus   0.61519726
 :earth   1.0
 :mars    1.8808158
 :jupiter 11.862615
 :saturn  29.447498
 :uranus  84.016846
 :neptune 164.79132})

(defn per-planet-age
  [seconds planet]
  (-> seconds
    (/ earth-year-seconds)
    (/ (planet periods)))
)

(defn on-mercury
  "Returns someone's age on Mercury based on the given age in seconds"
  [seconds]
  ;; function body
  (per-planet-age seconds :mercury)
  )

(defn on-venus
  "Returns someone's age on Venus based on the given age in seconds"
  [seconds]
  ;; function body
  (per-planet-age seconds :venus)
  )

(defn on-earth
  "Returns someone's age on Earth based on the given age in seconds"
  [seconds]
      (per-planet-age seconds :earth)
  )

(defn on-mars
  "Returns someone's age on Mars based on the given age in seconds"
  [seconds]
  (per-planet-age seconds :mars)
  )

(defn on-jupiter
  "Returns someone's age on Jupiter based on the given age in seconds"
  [seconds]
  ;; function body
  (per-planet-age seconds :jupiter)
  )

(defn on-saturn
  "Returns someone's age on Saturn based on the given age in seconds"
  [seconds]
  (per-planet-age seconds :saturn)
  )

(defn on-uranus
  "Returns someone's age on Uranus based on the given age in seconds"
  [seconds]
  (per-planet-age seconds :uranus)
  )

(defn on-neptune
  "Returns someone's age on Neptune based on the given age in seconds"
  [seconds]
  (per-planet-age seconds :neptune)
  )
