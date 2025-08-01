module sieve
  implicit none

contains

  function primes(limit) result(array)
    integer, intent(in) :: limit
    integer, allocatable :: array(:)
    integer, allocatable :: mark(:)
    integer:: i,j,nprimes
    
    if (limit < 2) then
      allocate(array(0))
      return
    end if
    
    allocate(mark(2:limit))
    mark = 1
    
    do i=2,int(sqrt(real(limit)))
      do j=i*i,limit,i
        if (mark(i)==1) then
          mark(j)=0
        end if
      end do
    end do
    
    nprimes=count(mark==1)
    allocate(array(nprimes))
    j=0
    do i=2,limit
      if (mark(i)==1) then
        j=j+1
        array(j)=i
      end if
    end do
  end function primes

end module sieve
