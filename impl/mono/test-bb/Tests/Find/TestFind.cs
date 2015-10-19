using sb.Core.Utils;
using sb.TestBB.Utils;

namespace sb.TestBB.Tests.Find
{
    public class TestFind : Test
    {
        public TestFind(App app)
            : base(app)
        {
        }

        public override void Run()
        {
            WriteStart();
            Fs.RunProcess("sb", "find", Output);
            if (TextUtils.ListContains(Output, "jekyll"))
            {
                WriteFinish(true);
                return;
            }
            WriteFinish(false);
        }
    }
}