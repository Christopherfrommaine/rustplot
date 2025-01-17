
from matplotlib import pyplot as plt

# Data replaced in file
title = None
axes = True
rge = ((0, 60), (0, 30))
path = None

# Plot, as determined by plot type in the rust file
plt.scatter([43.86680495182227, 1.5506780540012954, 15.730237954351502, 13.121814640290545, 44.96030574595638, 59.77904137945709, 6.771899988589074, 2.9640032659188753, 27.634779414886705, 13.62243869801203, 9.323348249577501, 12.274509400100282, 38.26496535279338, 52.56462658770962, 3.793888661059981, 13.007853783129942, 55.04662846630049, 0.9015014075551964, 37.08927569243953, 47.11426157511291, 5.064385624574008, 31.122712273450123, 52.06645786651306, 21.06732680791632, 31.34653006985542, 15.829089517852406, 59.09783977293173, 40.3240166385262, 17.55373617210255, 2.0589771575398697, 11.613369053447089, 34.887585580478564, 12.974070811649554, 21.184513526545793, 38.06928302436458, 4.99096100093392, 14.75051115356862, 54.923848145556434, 36.601969895285215, 48.054102424437446, 50.47244702022502, 45.935942124936766, 21.125891176377863, 32.46894632675419, 17.233359392970492, 46.22362365113062, 10.434848244271873, 11.929125091675562, 16.907620348243597, 16.881252703962023, 50.62135470935562, 29.892714671801443, 46.44231223296835, 4.754483715437754, 45.441084041265995, 43.25106125539084, 10.345610016754257, 20.685520226928688, 53.07787755467828, 12.049832970482456, 29.85467933020468, 49.395197253909224, 47.2084446813693, 52.96777329889607, 27.033500349340414, 22.85589985908981, 37.01686535259532, 58.75018329423624, 3.334106788069593, 6.8772877169120905, 1.7484689008112397, 1.5626428703473305, 23.896722920090305, 20.274226147182624, 14.891609089056036, 8.469184301325328, 38.180554455003005, 0.07725672079865742, 44.38905621474263, 39.72164156055876, 34.161276050537886, 57.04506458071516, 5.16312015956836, 54.69076121440145, 8.216062582888298, 51.12697866628801, 29.04170165319094, 30.08814100486431, 25.632904126528395, 34.043338776778256, 8.225537995853154, 26.876446271047595, 39.79963474579734, 8.094643429958555, 55.620290086874114, 35.34289274119088, 3.1030168478819675, 3.332331919860909, 6.227420462314619, 8.77912617194633], [23.20380553059714, 17.524777858086004, 23.15960717240462, 23.811403700074084, 29.0681324400717, 4.181292918155471, 28.945070989364083, 12.771061987829862, 27.727879706242856, 16.77996886239641, 25.06498660231699, 15.093878489516241, 14.365482398407217, 26.119811810508345, 29.606717030461237, 29.311601909135636, 20.316104414324908, 2.07664888458313, 18.413076233357398, 19.976656564918443, 2.5189359170582093, 22.58813856770993, 2.6281455450162228, 5.620562033014476, 29.30030313290053, 7.504636759893226, 19.980390002015366, 23.753644290545317, 19.03409273225886, 2.507671554923554, 19.67610614896248, 5.6029037111007085, 9.558391366583042, 16.70368145416576, 0.5416406676017038, 9.715901726321, 1.1635628974341228, 13.256440753510361, 1.4283792122567585, 24.41348706287927, 27.47652113771103, 26.39180776240608, 26.724661022555832, 3.2045928012691594, 10.835640385224268, 25.72803780932794, 15.46867517990558, 3.214464060037876, 9.703938320606857, 9.342595421153826, 26.144330187009224, 16.692638472051037, 15.63838039455276, 29.346198053683572, 6.37428067078551, 29.594487829271802, 7.599299383960185, 8.507191671144845, 17.96207051834144, 16.101063518859657, 3.8484294697335653, 9.348788466321576, 19.569248038661673, 17.399963487840168, 6.458302745118836, 16.721773937846873, 7.54268480299875, 2.0314834291064954, 25.085326036422856, 8.143819583660374, 19.587322910055477, 0.8891015187913354, 6.645456431036047, 26.267038186818013, 21.27617329833782, 19.81392103115811, 26.008978107571043, 10.972248648859932, 1.1779288867035187, 9.156896270177546, 15.917573877448461, 19.566493348387127, 21.006289916272685, 20.606964503967134, 18.417267959862556, 19.021347338732998, 16.32096354871038, 2.6144991874606327, 14.673735547809217, 18.404635094100655, 1.3233710716913283, 3.0182416173384574, 2.6032854403832095, 19.583862997583857, 10.285212027665105, 17.394569296825065, 20.900502156934536, 0.042262969749831125, 22.325946817757295, 27.689089272030614])

# Automatic
if title is not None:
    plt.suptitle(title)

if axes is not None:
    if axes:
        pass
    else:
        plt.cla()

if rge is not None:
    plt.xlim(rge[0])
    plt.ylim(rge[1])

if path is not None:
    plt.savefig(path)
else:
    plt.show()