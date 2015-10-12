using System;
using System.Diagnostics;
using sb.App;

namespace sb
{
    class Program
    {
        static void Main(string[] args)
        {
            try
            {
                new Args(args).Command.Run();
                Console.WriteLine("Done");
            }
            catch (Exception ex)
            {
                Console.WriteLine(ex); //todo:logs
                Debugger.Break(); 
            }
        }        
    }
}
