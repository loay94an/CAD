# CADCraft Android — فهرس مصادر أوامر المحرك

**المستودع:** `loay94an/CAD`  
**الفرع:** `feat/android-phone-tablet`  
**المرجع:** `8198386f977afec8ee19b37a0f8f7949f1a9a546`  
**الغرض:** جرد المصدر الصريح لأسماء الأوامر المسجلة عبر `CommandSpec::new("...")` في وحدات `crates/engine/src/cmd`.

## حدود هذا الفهرس

- هذا فهرس لمواضع التسجيل النصي، وليس قائمة نهائية مؤكدة بكل الأوامر التي قد تظهر وقت التشغيل.
- لا يحسب أسماء الأوامر المنشأة ديناميكياً أو aliases التي تضيفها builders أو UI-only actions.
- لا يثبت أن الأمر يعمل على Android؛ ذلك يحتاج تتبع handler واعتمادياته واختبار end-to-end.
- أسماء الأوامر تحفظ كما وردت في المصدر، بما في ذلك النقطة والشرطة والبادئات.

## الملخص

- الملفات المفحوصة: **40**
- التصريحات الصريحة المباشرة: **387**
- أسماء مكررة بين الملفات: **0**
- الملف `clipboard.rs` لا يحتوي تصريحاً مباشراً من هذا النمط؛ قد يحتوي helpers مستخدمة من أوامر أخرى.

## الفهرس حسب الملف

### `cmd/file.rs` — 9

`new`, `open`, `close`, `closeall`, `qsave`, `saveas`, `document.switch`, `document.bytes`, `script`

### `cmd/audit.rs` — 3

`audit`, `recover`, `drawingrecovery`

### `cmd/edit.rs` — 15

`u`, `undo`, `redo`, `mredo`, `cutclip`, `cutbase`, `copyclip`, `copybase`, `pasteclip`, `pasteblock`, `pasteorig`, `erase.selection`, `selectall`, `ai_deselect`, `select`

### `cmd/qselect.rs` — 2

`qselect`, `qselect.info`

### `cmd/selecttools.rs` — 4

`selectsimilar`, `isolateobjects`, `hideobjects`, `unisolateobjects`

### `cmd/view.rs` — 22

`zoom`, `zoom.realtime`, `zoom.dynamic`, `zoom.previous`, `zoom.window`, `zoom.in`, `zoom.out`, `zoom.all`, `zoom.extents`, `zoom.object`, `pan`, `-pan`, `pan.left`, `pan.right`, `pan.up`, `pan.down`, `regen`, `regenall`, `redraw`, `view.set`, `view.get`, `layout.set`

### `cmd/ucs.rs` — 7

`ucs`, `ucs.world`, `ucs.origin`, `ucs.3point`, `ucs.view`, `ucs.z`, `ucsicon`

### `cmd/draw.rs` — 16

`line`, `pline`, `circle`, `arc`, `rectang`, `polygon`, `ellipse`, `point`, `point.multiple`, `xline`, `ray`, `spline`, `donut`, `solid`, `text`, `mtext`

### `cmd/draw2.rs` — 28

`arc.sce`, `arc.sca`, `arc.scl`, `arc.sea`, `arc.sed`, `arc.ser`, `arc.cse`, `arc.csa`, `arc.csl`, `arc.continue`, `circle.cd`, `circle.2p`, `circle.3p`, `circle.ttr`, `circle.ttt`, `ellipse.axis`, `ellipse.arc`, `divide`, `measure`, `revcloud`, `revcloud.polygonal`, `revcloud.freehand`, `wipeout`, `3dpoly`, `helix`, `spline.cv`, `centermark`, `centerline`

### `cmd/annotate.rs` — 23

`dimlinear`, `dimaligned`, `dimradius`, `dimdiameter`, `dimangular`, `dimarc`, `dimordinate`, `dimcontinue`, `dimbaseline`, `qdim`, `dim`, `mleader`, `leader`, `dimstyle.update`, `dimoverride`, `dimreassociate`, `dimdisassociate`, `dimtedit.home`, `dimtedit.angle`, `dimtedit.left`, `dimtedit.center`, `dimtedit.right`, `dimtedit`, `dimspace`

### `cmd/dimstyles.rs` — 2

`dimoverride.clear`, `dimstyle.saveas`

### `cmd/dimstyle_cli.rs` — 2

`-dimstyle`, `dimstyle.overrides`

### `cmd/mleader_edit.rs` — 5

`mleaderedit`, `mleaderedit.add`, `mleaderedit.remove`, `mleaderalign`, `mleadercollect`

### `cmd/annoscale.rs` — 9

`scalelistedit`, `objectscale`, `aiobjectscaleadd`, `aiobjectscaleremove`, `annoreset`, `annoupdate`, `annovisible.current`, `annovisible.all`, `annovisible.toggle`

### `cmd/hatch.rs` — 7

`hatch`, `gradient`, `boundary`, `region`, `hatchedit`, `hatchgenerateboundary`, `hatchpick`

### `cmd/overkill.rs` — 1

`overkill`

### `cmd/blocks.rs` — 14

`block`, `-block`, `insert`, `blockspalette`, `minsert`, `attdef`, `base`, `blocks.dynamic`, `blocks.reference`, `blocks.reference.set`, `blocks.reference.grips`, `blocks.reference.grip`, `purge`, `blocks.list`

### `cmd/modify.rs` — 17

`erase`, `oops`, `move`, `copy`, `rotate`, `scale`, `mirror`, `stretch`, `offset`, `trim`, `extend`, `fillet`, `chamfer`, `explode`, `break`, `breakatpoint`, `join`

### `cmd/draworder.rs` — 11

`draworder`, `draworder.front`, `draworder.back`, `draworder.above`, `draworder.under`, `texttofront`, `texttofront.text`, `texttofront.dimensions`, `texttofront.leaders`, `texttofront.all`, `hatchtoback`

### `cmd/array.rs` — 4

`array`, `arrayrect`, `arraypolar`, `-array`

### `cmd/modify2.rs` — 12

`pedit`, `splinedit`, `lengthen`, `align`, `blend`, `reverse`, `ncopy`, `mocoro`, `chspace`, `flatten`, `arraypath`, `textedit`

### `cmd/groups.rs` — 4

`group`, `ungroup`, `groupedit`, `groups.list`

### `cmd/gripcmds.rs` — 7

`grip.move`, `grip.stretch`, `grip.rotate`, `grip.scale`, `grip.mirror`, `grip.option`, `grip`

### `cmd/layer.rs` — 24

`layer`, `layer.new`, `layer.set`, `layer.current`, `layer.delete`, `laymcur`, `laymch`, `laycur`, `layiso`, `layuniso`, `layfrz`, `layoff`, `laylck`, `layulk`, `layon`, `laythw`, `layerp`, `layerstate.save`, `layerstate.restore`, `layerstate.list`, `layerstate.delete`, `layerstate.rename`, `laymrg`, `laydel`

### `cmd/layout.rs` — 22

`layout`, `layout.new`, `layout.delete`, `layout.rename`, `layout.copy`, `layout.list`, `layout.saveas`, `mview`, `mview.polygonal`, `vpclip`, `vports`, `vports.1`, `vports.2`, `vports.3`, `vports.4`, `viewport.set`, `vplayer`, `pagesetup`, `plot`, `exportpdf`, `mspace`, `pspace`

### `cmd/exchange.rs` — 8

`import`, `export`, `preview`, `publish`, `pdfbook`, `attach`, `share`, `3dorbit`

### `cmd/image.rs` — 2

`imageattach`, `image`

### `cmd/props.rs` — 32

`properties`, `properties.set`, `chprop`, `change`, `properties.describe`, `matchprop`, `color`, `linetype`, `hatch.patterns`, `-linetype`, `lweight`, `ltscale`, `units`, `-units`, `thickness`, `limits`, `style`, `style.list`, `style.rename`, `style.delete`, `style.current`, `dimstyle`, `dimstyle.dimension`, `dimstyle.list`, `dimstyle.rename`, `dimstyle.delete`, `dimstyle.current`, `dimstyle.override`, `tablestyle`, `mleaderstyle`, `ddptype`, `rename`

### `cmd/inquiry.rs` — 15

`dist`, `id`, `list`, `area`, `time`, `status`, `drawing.inspect`, `entities`, `shapes`, `usage`, `loops`, `texts`, `countlist`, `counttable`, `count`

### `cmd/settings.rs` — 17

`ortho`, `grid`, `snap`, `polar`, `otrack`, `dynmode`, `lwdisplay`, `isodraft`, `isoplane`, `transparencydisplay`, `qpmode`, `selectioncycling`, `osnap`, `dsettings`, `setvar`, `getvar`, `sysvars`

### `cmd/options.rs` — 1

`options`

### `cmd/interface.rs` — 2

`menubar`, `toolbar`

### `cmd/utility.rs` — 7

`cal`, `quickcalc.pick`, `find`, `measuregeom`, `radius.measure`, `angle.measure`, `volume.measure`

### `cmd/table.rs` — 8

`table`, `table.set`, `table.insertrow`, `table.deleterow`, `table.insertcol`, `table.deletecol`, `table.merge`, `table.unmerge`

### `cmd/tolerance.rs` — 1

`tolerance`

### `cmd/txt2mtxt.rs` — 1

`txt2mtxt`

### `cmd/constraints.rs` — 16

`geomconstraint`, `autoconstrain`, `constraintbar`, `constraintbar.showall`, `constraintbar.hideall`, `dimconstraint`, `dclinear`, `dcdisplay`, `dcdisplay.showall`, `dcdisplay.hideall`, `delconstraint`, `parameters`, `constraintsettings`, `constraintinfer`, `constraints.inspect`, `constraints.dof`

### `cmd/lisp.rs` — 2

`appload`, `lisp`

### `cmd/file/compare.rs` — 4

`compare`, `compare.next`, `compare.previous`, `compareclose`

### `cmd/clipboard.rs` — 0

لا توجد تصريحات مباشرة.

## الخطوة التالية لإغلاق M0

1. استخراج `command_specs()` من registry وقت الاختبار، لا الاعتماد على regex وحده.
2. ضم `alias_specs()` و`UI_COMMANDS` وأوامر LISP/التحميل الديناميكي.
3. لكل ID: تسجيل handler الفعلي، parameters، interactive state، enabled condition، menu/toolset mapping.
4. ربط اختبارات الوحدة/التكامل بكل أمر، وتحديد ما يحتاج Android adapter.
5. تحويل هذا الفهرس إلى JSON/CSV آلي حتى لا يتقادم عند إضافة أوامر جديدة.

**تنبيه:** لا تفسر وجود ID في هذا الفهرس على أنه دعم Android أو نجاح اختبار الأمر.


## التسجيلات الإضافية التي لا تظهر في ملفات الوحدات الأربعين

### PGP / Alias commands

يضيف `crates/engine/src/cmd.rs::command_specs()` أيضاً `file::alias_specs()`، وهي إعادة تصدير `pgp::specs` من `crates/engine/src/cmd/file/pgp.rs`:

- `aliasedit`
- `reinit`

بالتالي، اختبار CI الفعلي كشف أن `command_specs()` يعيد **477 CommandSpec** على بيئة Linux. الرقم 389 هو مجموع التصريحات المباشرة المرصودة (387) وPGP aliases (2)، وليس العدد الكامل لسجل runtime. توجد تسجيلات إضافية لا يلتقطها فهرس `CommandSpec::new` النصي. اختبار العدد والتفرّد يعتمد الآن 477، ويبقى استخراج IDs الفعلية وربطها بالمعالجات والمعاملات والاختبارات مطلوباً لإغلاق M0.

### UI-only commands

تحتوي `crates/ui-egui/src/menus.rs::UI_COMMANDS` على 60 إجراءً لا تمر عبر `CommandSpec` في محرك الأوامر:

```text
ui.spacemouse
ui.spacemouse.calibrate
ui.spacemouse.navigate
ui.dock
ui.dock.reset
ui.open
ui.newtemplate
ui.saveas
ui.sample
ui.toggle.toolsets
ui.panel.properties
ui.panel.layers
ui.toggle.quickcalc
ui.panel.quickcalc
ui.toggle.systemcursor
ui.animations.system
ui.animations.on
ui.animations.off
ui.animations
ui.toggle.toolbar
ui.toggle.filetabs
ui.toggle.statusbar
ui.toggle.cmdline
ui.toggle.viewcube
ui.toggle.ucsicon
ui.toggle.ucsicon.origin
ui.theme.system
ui.theme.light
ui.theme.dark
ui.theme
ui.saveformat.dxf
ui.saveformat.dwg
ui.saveformat
ui.saveversion.2018
ui.saveversion.2013
ui.saveversion.2010
ui.saveversion.2007
ui.saveversion.2004
ui.saveversion.2000
ui.saveversion
ui.toggle.menubar
ui.hidepalettes
ui.resetpalettes
ui.start
ui.dialog.layers
ui.dialog.blocks
ui.dialog.qselect
ui.dialog.parameters
ui.dialog.dsettings
ui.updates.check
ui.dialog.about
ui.dialog.commands
ui.toggle.history
ui.cmdline.lines
ui.cmdline.clear
ui.dialog.language
ui.dialog.find
ui.shortcuts
ui.language
ui.quit
```

**المجموع المفهرس الآن:** 389 أمر محرك + 60 إجراء واجهة فقط. هذا لا يشمل أسماء aliases التي يضيفها المستخدم في ملفات PGP؛ فهي بيانات تشغيل وليست IDs جديدة في registry.

## اختبارات تثبيت الفهرس

أضيفت اختبارات تمنع تغيّر العدد أو تكرار IDs دون تحديث الفهرس:
- `crates/engine/src/cmd.rs::registry_tests::command_registry_has_389_unique_ids`
- `crates/ui-egui/src/menus.rs::ui_command_inventory_tests::ui_only_command_ids_are_unique_and_match_the_source_index`

هذه الاختبارات تثبت العدد والتفرّد عند تشغيلها، لكنها لا تثبت جاهزية كل أمر على Android أو اكتمال handler mapping.
