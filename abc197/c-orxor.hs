{-
import Data.Bits
import Control.Applicative
import Data.List

-- letの使い方でエラー(変数束縛)

main = do
    n <- readLn :: IO Int
    a_s <- map read . words <$> getLine
    putStrLn $ show (a_s !! 0 + 0)

    -- let ex = [[[[1,2],[3,4]], [[1,2,3], [4]]] -- 分割法不明 (ここがうまくいけばACなはず)

    -- let or = map calcOR [[1], [2,3,4]]   -- --> [1, 7] 

    let ora = [map calcOR u | u <- a_s] 

    let xor = [map calcXOR v | v <- ora]
    let ans = reverse $ sort xor

    putStrLn $ show (ans !! 0 + 0)

-- 配列を受け取り、全体のOR, XORを計算する関数

calcOR [] = 0
calcOR (x:xs) = x .|. calcOR xs

calcXOR [] = 0
calcXOR (x:xs) = x `xor` calcXOR xs

-}

import Data.Bits
import Control.Applicative
import Data.List

-- [[[[1,2],[3,4]], [[1,2,3], [4]]]

calcOR :: [Int] -> Int
calcOR [] = 0
calcOR (x:xs) = x .|. calcOR xs

calcXOR :: [Int] -> Int
calcXOR [] = 0
calcXOR (x:xs) = x `xor` calcXOR xs

-- solve :: [[[Int]]] -> [Int]
-- solve k = reverse $ sort [map calcXOR v | v <- [map calcOR u | u <- k] ]
