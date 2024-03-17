{-# LANGUAGE OverloadedStrings #-}

import Data.Text (Text)
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Text.Regex.TDFA ((=~), (=~~))

main :: IO ()
main = do
  s <- TIO.getLine
  let transS = s =~ "\\[^|]*\\|" :: Text
  let resutl = s =~~ "\\|[^|]*\\|" :: Maybe Text
  case result of
    Just match -> TIO.putStrLn $ T.replace match "" s
    Nothing -> TIO.putStrLn s
