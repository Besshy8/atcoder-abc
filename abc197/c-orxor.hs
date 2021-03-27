import Data.Bits
import Control.Applicative
import Data.List

main = do
    n <- readLn :: IO Int
    {-
    let ex = [[[[1,2],[3,4]], [[1,2,3], [4]]]
        ora = [map calcOR u | u <- ex]
        xor = [map calcXOR v | v <- ora]
        ans = reverse $ sort xor
    putStrLn $ show (ans !! 0)
    -}
    
{-
import Data.Bits
import Control.Applicative
import Data.List

-- letの使い方でエラー(変数束縛)

main = do
    n <- readLn :: IO Int
    -- a_s <- map read . words <$> getLine

    let ex = [[[[1,2],[3,4]], [[1,2,3], [4]]] -- 分割法不明 (ここがうまくいけばACなはず)

    -- let or = map calcOR [[1], [2,3,4]]   -- --> [1, 7] 

    let ora = [map calcOR u | u <- ex] 

    let xor = [map calcXOR v | v <- ora]
    let ans = reverse $ sort xor

    putStrLn $ show (ans !! 0)

-- 配列を受け取り、全体のOR, XORを計算する関数

calcOR [] = 0
calcOR (x:xs) = x .|. calcOR xs

calcXOR [] = 0
calcXOR (x:xs) = x `xor` calcXOR xs


-}