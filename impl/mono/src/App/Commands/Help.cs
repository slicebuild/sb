using System;
using cb.App.Messages;

namespace cb.App.Commands
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