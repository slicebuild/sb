using System;
using sb_bbt.App;

namespace sb_bbt
{
    class Program
    {
        static void Main(string[] args)
        {
            try
            {
                new Args().Run();
                Console.WriteLine();
                Console.WriteLine("Done!");
            }
            catch (Exception ex)
            {
                Console.WriteLine(ex);
                Console.Read();
            }
        }
    }
}
