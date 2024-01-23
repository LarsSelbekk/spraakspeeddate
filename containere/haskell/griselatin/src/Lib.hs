module Lib
  ( pigLatin,
  piggifyWordWithPunctuation,
  piggifyLetters,
  pigMain,
  splitAtPunctuation,
  includes
  )
where

import Data.Char (isUpper, toLower, toUpper)
import Prelude

pigMain :: IO ()
pigMain = do
  inp <- getLine
  putStrLn (pigLatin inp)

pigLatin :: String -> String
pigLatin sentence = unwords (map piggifyWordWithPunctuation (words sentence))

piggifyLetters :: String -> String
piggifyLetters inp = transformed
  where
    consonants = "bcdfghjklmnpqrstvwxz"
    pigify word = case word of
      first : rest ->
        if includes (toLower first) consonants then
          case rest of
            newFirst : middle -> if isUpper first then
              toUpper newFirst : map toLower middle ++ [toLower first] ++ "ay"
              else rest ++ [toLower first] ++ "ay"
            [] -> first : "ay"
        else first : map toLower rest ++ "way"
      [] -> ""
    transformed = pigify inp

splitAtPunctuation :: String -> (String, String)
splitAtPunctuation word = (letterPart, punctuationPart)
  where
    punctuation = ",.-!?;:\""
    split letters unknown = case unknown of
      candidate : rest ->
        if includes candidate punctuation
          then (letters, unknown)
          else split (letters ++ [candidate]) rest
      [] -> (letters, [])
    (letterPart, punctuationPart) = split [] word

piggifyWordWithPunctuation :: String -> String
piggifyWordWithPunctuation word = pigified
  where
    (letterPart, punctuationPart) = splitAtPunctuation word
    pigified = piggifyLetters letterPart ++ punctuationPart

includes :: Char -> String -> Bool
includes needle haystack = case haystack of
  letter : rest -> (needle == letter) || includes needle rest
  [] -> False
