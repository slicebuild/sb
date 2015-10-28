using System;
using System.Diagnostics;
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
                Debug.WriteLine(ex);
                Console.WriteLine(ex); //todo:logs                
            }
        }    
        
        // make, test dir
        // major for fetch
        // args reg command, options
        // download zip and unpack
        // fetch if newer, unstable, force
        // todo: dock|run (tti, cur dir, cur user)  
        // scratch itch  
        // plugins (formatters)
        // web station (login/fiddle/display) verify (show current/last/next verified)    
        // check sums for known files
        // optimize from image layers 
        // chain stdin/out, check Console

    }
}
