using System.Collections.Generic;
using System.IO;
using cb.Utils;

namespace cb.App.Commands
{
    public class Test : Make, ICommand
    {
        public Test(Args args)
            : base(args)
        {
        }

        void ICommand.Run()
        {
            Args.SetOption(Args.Options.Format, Args.OptionValues.FormatDocker);
            var layer = MakeLayer();
            var path = MakePath(layer);
            Write(layer, path);

            var stdout = new List<string>();
            Fs.RunProcess("docker", $"build -t wb-test {Path.GetDirectoryName(path)}", stdout);

            if (stdout.Count > 0)
            {
                var items = stdout[stdout.Count - 1].Split(" ");
                if (items.Length >= 3 && items[0].StartsWith("Success"))
                {
                    var id = items[2];
                    //todo: use the generated image id
                }
            }
        }
    }
}