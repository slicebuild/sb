using System.IO;
using sb_bbt.Utils;

namespace sb_bbt.Tests.Find
{
    public class TestFindRandom : Test
    {
        public TestFindRandom(App app)
            : base(app)
        {
        }

        public override void Run()
        {
            WriteStart();
            //todo: pick up a latest slice and use it as the param to find
            Fs.RunProcess("sb", "find ruby", Output);
            if (Text.ListContains(Output, "ruby"))
            {
                WriteFinish(true);
                return;
            }
            WriteFinish(false);
        }
    }
}