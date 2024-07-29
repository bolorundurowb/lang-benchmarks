using System.Diagnostics;

var primesFound = 0;
var largestPrime = 0L;

var stopwatch = new Stopwatch();
stopwatch.Start();
Parallel.For(0, int.MaxValue, (i) =>
{
    if (IsPrime(i))
    {
        Interlocked.Increment(ref primesFound);
        largestPrime = i;
    }
});

stopwatch.Stop();
Console.WriteLine(
    $"{primesFound} prime numbers found in {stopwatch.ElapsedMilliseconds}ms. Largest prime: {largestPrime}");


static bool IsPrime(long number)
{
    if (number <= 1)
        return false;

    if (number <= 3)
        return true;

    if (number % 2 == 0 || number % 3 == 0)
        return false;

    var i = 5;
    while (i * i <= number)
    {
        if (number % i == 0 || number % (i + 2) == 0)
            return false;

        i += 6;
    }

    return true;
}

