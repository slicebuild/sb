using System;
using sb.App.Messages;

namespace sb.App.Commands
{
    public class Help : ICommand
    {
        public Help(Args args)
        {
        }

        public void Run()
        {
            Console.WriteLine(new HelpMsg().TransformText());
        } 
    }
}