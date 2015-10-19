using System;
using sb.Core.App.Messages;

namespace sb.Core.App.Commands
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