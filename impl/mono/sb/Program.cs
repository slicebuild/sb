using System;
using sb.Core.App;

namespace sb
{
    class Program
    {
        static void Main(string[] args)
        {
            try
            {
                new Args(args).Command.Run();
            }
            catch (Exception ex)
            {
                Console.WriteLine(ex); //todo:logs                
            }
        }    
        

        // args reg command, options
        // download zip and unpack
        // fetch if newer, unstable, force
        // handle versions everywhere
        //todo: dock|run (tti, cur dir, cur user)  
        // scratch your itch movement 
        // plugins 
        // web station      
        // check sums for known files
        // optimize from image layers 
    }
}
