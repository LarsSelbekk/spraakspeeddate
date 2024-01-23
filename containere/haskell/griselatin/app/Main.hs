module Main (main) where

import Lib

main :: IO ()
main = 
  do
    inp <- getLine
    putStr (pigLatin inp)

