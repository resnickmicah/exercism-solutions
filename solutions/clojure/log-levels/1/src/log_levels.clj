(ns log-levels
  (:require [clojure.string :as str]))



(defn message
  "Takes a string representing a log line
   and returns its message with whitespace trimmed."
  [s]
  (-> s 
    (clojure.string/split #" ")
    rest
    (#(clojure.string/join " " %))
    clojure.string/trim)
  )

(defn log-level
  "Takes a string representing a log line
   and returns its level in lower-case."
  [s]
  (-> s
    (clojure.string/split #" ")
    first
    (#(re-find #"[A-Z]+" %))
    clojure.string/lower-case)
  )

(defn reformat
  "Takes a string representing a log line and formats it
   with the message first and the log level in parentheses."
  [s]
  (str (message s) " (" (log-level s) ")")
  )
