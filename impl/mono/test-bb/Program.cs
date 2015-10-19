using System;
using sb_bbt;

namespace sb.TestBB
{
    class Program
    {
        static void Main(string[] args)
        {
            try
            {
                new App().Run();
                Console.WriteLine();
                Console.WriteLine("Done! Press any key to exit");
                Console.ReadLine();
            }
            catch (Exception ex)
            {
                Console.WriteLine(ex);
                Console.Read();
            }
        }
    }
}
