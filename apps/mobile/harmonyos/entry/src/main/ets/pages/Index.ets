import { taskpool } from '@kit.ArkTS';

import ydx_harmonyos from 'libydx_harmonyos.so';

@Concurrent
function runTest(): number {
  return ydx_harmonyos.ydx_harmonyos_performance_test();
}

@Entry
@Component
struct Index {
  @State state: string = 'Run performance test';

  build() {
    RelativeContainer() {
      Column({ space: 24 }) {

        Text('YDX')
          .fontSize($r('app.float.page_title'))
          .fontWeight(FontWeight.Bold)

        Row({ space: 20 }) {
          Image($r('app.media.ohos'))
            .width(100)
            .height(100)
            .objectFit(ImageFit.Cover)

          Image($r('app.media.rust'))
            .width(100)
            .height(100)
            .objectFit(ImageFit.Cover)
        }
        .backgroundColor($r('app.color.carg_background'))
        .padding(16)
        .borderRadius(20)

        Button(this.state)
          .enabled(this.state !== 'Loading...')
          .onClick(() => {
            taskpool.execute(runTest)
              .then((value: Object) => {
                this.state = `The test was performed: ${value}ms`;
              })
              .catch((error: Error) => {
                this.state = `Error: ${error.message}`;
              });
            this.state = 'Loading...'
          })

        Text('Click to test the performance of the\nRust C library.')
          .fontSize($r('app.float.page_text'))
          .textAlign(TextAlign.Center)
      }
      .alignRules({
        center: { anchor: '__container__', align: VerticalAlign.Center },
        middle: { anchor: '__container__', align: HorizontalAlign.Center }
      })
      .padding(16)

    }.height('100%').width('100%')
  }
}
