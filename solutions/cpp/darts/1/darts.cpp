#include "darts.h"
#include <cmath>
namespace darts {
    int score(double x, double y){
        double r{sqrt(pow(x,2)+pow(y,2))};
        if (r <= 1){
            return 10;
        }else if (r <= 5){
            return 5;
        }else if (r <= 10){
            return 1;
        }else{
          return 0; 
        };
    }
} // namespace darts